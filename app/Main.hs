module Main where

import qualified Control.Exception as Exception
import GHC.IO.Exception (IOException (IOError))
import Lexer (lexText)
import Parser (parseText)
import qualified System.Environment as Env
import qualified System.IO.Error as Error

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
    withErrorHandling $
        handleArgs
            >>= \arg ->
                case arg of
                    Left err ->
                        putStrLn $ "Error processing file: " <> err
                    Right fname ->
                        readFile fname >>= putStrLn
  where
    withErrorHandling :: IO () -> IO ()
    withErrorHandling ioAction = Exception.catch ioAction handleErr
    handleErr :: IOError -> IO ()
    handleErr e = putStrLn "Error reading file: " >> print e

-- Lexer.lexText
-- Parser.parseText
