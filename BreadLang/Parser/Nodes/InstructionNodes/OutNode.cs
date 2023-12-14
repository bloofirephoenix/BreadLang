using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class OutNode : Node
{
    public override void Populate(Parser parser)
    {
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
        return "OUT";
    }

    public override int GetSize()
    {
        return 1 + (Children.Count > 0 ? 1 : 0);
    }
}