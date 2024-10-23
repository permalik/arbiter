module Main where

-- import qualified System.Environment as Env
import Lexer (lexText)
import Parser (parseText)

main :: IO ()
main = do
    putStrLn "Hello, Haskell!"
    Lexer.lexText
    Parser.parseText
