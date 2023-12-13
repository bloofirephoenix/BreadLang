using BreadLang.Tokens;

namespace BreadLang;

public class ErrorHandler
{
    public static ErrorHandler? Instance;

    private string _source;

    public ErrorHandler(string source)
    {
        if (Instance != null)
        {
            throw new Exception("Cannot have multiple error handler instances!");
        }

        Instance = this;
        _source = source;
    }

    public void Error(int line, string where, string message)
    {
        Console.WriteLine($"[line {line}] Error {message}: {where}");
        Environment.Exit(64);
    }

    public void Error(Token token, string message)
    {
        Console.WriteLine($"[line {token.Line}] Error {message}: {token.Lexeme}");
        Environment.Exit(64);
    }
}