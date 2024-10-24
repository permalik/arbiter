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
    handleArgs >>= displayMessage
  where
    displayMessage parsedArgument =
        case parsedArgument of
            Left errMessage ->
                putStrLn $ "Error: " <> errMessage
            Right fileName ->
                putStrLn $ "Opening File:" <> fileName

-- Lexer.lexText
-- Parser.parseText
