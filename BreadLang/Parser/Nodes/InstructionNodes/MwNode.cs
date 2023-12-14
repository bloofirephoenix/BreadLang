using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class MwNode : Node
{
    public override void Populate(Parser parser)
    {
        PopulateAndAdd(new RegisterNode(), parser);

        parser.Expect(TokenType.Comma);

        if (parser.Check(TokenType.Register))
            PopulateAndAdd(new RegisterNode(), parser);
        else if (parser.Check(TokenType.Number))
            PopulateAndAdd(new NumberNode(NumberNode.Type.Immediate8), parser);
        else
            ErrorHandler.Instance!.Error(parser.Current(), "Expected register or number");
    }

    public override void Compile(Compiler compiler)
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return "Mw";
    }

    public override int GetSize()
    {
        return 2;
    }
}