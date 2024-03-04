using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.Placeholders;

public abstract class PlaceholderNode : Node
{
    protected Token? Token;

    public override void Populate(Parser parser)
    {
        Token = parser.Expect(TokenType.Identifier);
    }
}