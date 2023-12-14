using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes;

public class PlaceholderNode : Node
{
    public string Name { get; private set; } = "";

    private Token _token;

    public override void Populate(Parser parser)
    {
        _token = parser.Expect(TokenType.Identifier);
        Name = _token.Lexeme;
    }

    public override void Compile(Compiler compiler)
    {
        if (!compiler.Placeholders.ContainsKey(Name))
        {
            ErrorHandler.Instance!.Error(_token, $"Could not find {Name}");
            return;
        }
        compiler.Write(compiler.Placeholders[Name]);
    }

    public override string ToString()
    {
        return $"Placeholder(name={Name})";
    }

    public override int GetSize()
    {
        return 2;
    }
}