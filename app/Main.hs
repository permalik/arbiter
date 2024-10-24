module Main where

import Lexer (lexText)
import Parser (parseText)
import qualified System.Environment as Env

handleArgs :: IO (Either String FilePath)
handleArgs = parseArgs <$> Env.getArgs
  where
    parseArgs argumentList =
        case argumentList of
            [fname] -> Right fname
            [] -> Left "error: no arguments provided"
            _ -> Left "error: multiple files not supported"

main :: IO ()
main =
    handleArgs
        >>= \fnameOrError ->
            case fnameOrError of
                Left err ->
                    putStrLn $ "Error: " <> err
                Right fname ->
                    readFile fname >>= putStrLn

-- Lexer.lexText
-- Parser.parseText
