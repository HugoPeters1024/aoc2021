import qualified Data.Map as M
import Control.Monad.State.Lazy
import GHC.Settings (maybeRead)
import qualified Z3.Monad as Z
import Data.Maybe

main :: IO ()
main = do
    content <- lines <$> readFile "input2.txt"
    let initialState = (0, M.fromList [(X, Lit 0), (Y, Lit 0), (Z, Lit 0), (W, Lit 0)])
    let (_, vars) = snd <$> runState (mapM addInstr content) initialState
    let exprz = simplify $ vars M.! Z
    sol <- Z.evalZ3 $ exprToZ3 exprz
    print sol

    case sol of
      Nothing -> print "No solution"
      Just xs -> let 
        solState = M.fromList $ zip [0..13] (map fromIntegral xs)
        in do print $ mkFinalSolution xs
              putStrLn $ show xs ++ " => " ++ show (evalState (eval exprz) solState)

mkFinalSolution :: [Integer] -> Integer
mkFinalSolution xs = sum $ zipWith (\d e -> d * (10^e)) xs (reverse [0..13])

type Vars = M.Map Ident Expr
type S = (Int, Vars)

data Ident = X | Y | Z | W | I Int deriving (Eq, Ord)

data Op = Add | Mul | Div | Mod | Eql | Nql
data Expr = Lit Int | Var Ident | BinOp Op Expr Expr

instance Show Op where
    show Add = "+"
    show Mul = "*"
    show Div = "/"
    show Mod = "%"
    show Eql = "=="
    show Nql = "!="


instance Show Expr where
    show (Lit i) = show i
    show (Var i) = show i
    show (BinOp op l r) = "(" ++ show l ++ show op ++ show r ++ ")"

instance Show Ident where
    show X = "x"
    show Y = "y"
    show Z = "z"
    show W = "w"
    show (I i) = 'i':show i

parseIdent :: String -> Ident
parseIdent "x" = X
parseIdent "y" = Y
parseIdent "z" = Z
parseIdent "w" = W
parseIdent _   = error "shite"

evalOp :: Op -> Int -> Int -> Int
evalOp Add = (+)
evalOp Mul = (*)
evalOp Div = div
evalOp Mod = mod
evalOp Eql = \l r -> if l == r then 1 else 0
evalOp Nql = \l r -> if l == r then 0 else 1

eval :: Expr -> State (M.Map Int Int) Int
eval (Lit i) = pure i
eval (Var (I i)) = get >>= \map -> pure $ map M.! i
eval (Var _) = error "bruh"
eval (BinOp op l r) = eval l >>= \l -> eval r >>= \r -> pure $ evalOp op l r


exprToZ3 :: Expr -> Z.Z3 (Maybe [Integer])
exprToZ3 e = do
    vars <- mapM (\i -> Z.mkFreshIntVar ('i':show i)) [0..13]
    --vars <- mapM (\i -> Z.mkInteger 9) [0..13]
    let lookup :: M.Map Int Z.AST
        lookup = M.fromList (zip [0..13] vars)

    let binOp :: Op -> Z.AST -> Z.AST -> Z.Z3 Z.AST
        binOp Add l r = Z.mkAdd [l,r]
        binOp Mul l r = Z.mkMul [l,r]
        binOp Div l r = Z.mkDiv l r
        binOp Mod l r = Z.mkMod l r
        binOp Eql l r = do
            c <- Z.mkEq l r
            t <- Z.mkInteger 1
            f <- Z.mkInteger 0
            Z.mkIte c t f
        binOp Nql l r = do
            c <- Z.mkEq l r
            t <- Z.mkInteger 0
            f <- Z.mkInteger 1
            Z.mkIte c t f
    
    let trav :: Expr -> Z.Z3 Z.AST
        trav (Lit i) = Z.mkInteger (fromIntegral i)
        trav (Var (I i)) = pure $ lookup M.! i
        trav (Var _) = error "bro"
        trav (BinOp op l r) = trav l >>= \l -> trav r >>= \r -> binOp op l r

    _0 <- Z.mkInteger 0
    _1 <- Z.mkInteger 1
    _9 <- Z.mkInteger 9
    minSol <- Z.mkInteger 11111111111111
    maxSol <- Z.mkInteger 99999999999999

    let bounds :: Z.AST -> Z.Z3 Z.AST
        bounds var = do
            lower <- Z.mkLe _1 var
            higher <- Z.mkLe var _9
            Z.mkAnd [lower, higher]

    allBounds <- mapM bounds vars
    Z.assert =<< Z.mkAnd allBounds

    Z.assert =<< (trav e >>= Z.mkEq _0 >>= Z.simplify)

    --solution <- Z.mkAdd =<< zipWithM (\b e -> Z.mkInteger (10^e) >>= \r -> Z.mkMul [b, r]) vars (reverse [0..13])
    --Z.assert =<< Z.mkGe solution minSol
    --Z.assert =<< Z.mkGe maxSol solution
    
    fmap snd $ Z.withModel $ \m -> catMaybes <$> mapM (Z.evalInt m) vars
    --(Just x) <- fmap snd $ Z.withModel $ \m -> Z.evalInt m solution
    --pure ((:[]) <$> x)
    --(\x -> Just [fromIntegral x]) <$> Z.optimizeMaximize solution




simplify :: Expr -> Expr
simplify (BinOp op l r) = case (op, simplify l, simplify r) of
                            (_, Lit l, Lit r) -> Lit $ evalOp op l r
                            (Mul, Lit 0, _) -> Lit 0
                            (Mul, _, Lit 0) -> Lit 0
                            (Mul, Lit 1, r) -> r
                            (Mul, l, Lit 1) -> l
                            (Add, Lit 0, r) -> r
                            (Add, l, Lit 0) -> l
                            (Div, Lit 0, r) -> Lit 0
                            (Div, l, Lit 0) -> error "division by zero"
                            (Div, l, Lit 1) -> l
                            (Eql, l@(Lit n), r@(Var (I _))) -> if n > 9 || n < 1 then Lit 0 else BinOp Eql l r
                            (Eql, l@(Var (I _)), r@(Lit n)) -> if n > 9 || n < 1 then Lit 0 else BinOp Eql l r
                            (Eql, l@(BinOp Eql _ _), r@(Lit 1)) -> l
                            (Eql, BinOp Eql l r, Lit 0) -> BinOp Nql l r
                            (Mod, l@(Var (I _)), r@(Lit n)) -> if n > 9 then l else BinOp Mod l r
                            (Mod, _, Lit 0) -> error "mod by zero"
                            (Mod, _, Lit 1) -> Lit 1
                            (Mod, l@(BinOp Eql _ _), _) -> l
                            (op, l, r) -> BinOp op l r
simplify e = e

addInstr :: String -> State S ()
addInstr s = let
    binOp :: Op -> String -> String -> State S ()
    binOp op il ir = do
        l <- getVar il
        r <- getVar ir
        setVar il (BinOp op l r)

    in case words s of
               ["inp",i0] -> nextIdent >>= \varname -> setVar i0 (Var varname)
               ["add",il,ir] -> binOp Add il ir
               ["mul",il,ir] -> binOp Mul il ir
               ["div",il,ir] -> binOp Div il ir
               ["mod",il,ir] -> binOp Mod il ir
               ["eql",il,ir] -> binOp Eql il ir
               _ -> error "could not parse"


getVar :: String -> State S Expr
getVar ident = case maybeRead ident of
                 Just i -> pure (Lit i)
                 Nothing -> do
                    (_, vars) <- get
                    pure $ vars M.! parseIdent ident

setVar :: String -> Expr -> State S ()
setVar ident expr = do
    (i, vars) <- get
    put (i, M.insert (parseIdent ident) (simplify expr) vars)

nextIdent :: State S Ident
nextIdent = do
    (i, v) <- get
    put (i+1, v)
    pure $ I i

