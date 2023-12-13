using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes;

public class RegisterNode : Node
{
    public string Register { get; private set; }

    public override void Populate(Parser parser)
    {
        var token = parser.Expect(TokenType.Register);
        Register = token.Lexeme;
    }

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return $"Register({Register})";
    }
}