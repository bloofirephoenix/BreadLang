using BreadLang.Compiling;
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

    public override void Compile(Compiler compiler)
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return $"Register({Register})";
    }

    public override int GetSize()
    {
        throw new NotImplementedException(); // it is invalid to ask for the size of a register node.
    }
}