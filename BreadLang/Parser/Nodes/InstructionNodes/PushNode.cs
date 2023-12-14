using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class PushNode : Node
{
    public override void Populate(Parser parser)
    {
        Node node;
        if (parser.Peek().Type == TokenType.Number)
            node = new NumberNode(NumberNode.Type.Immediate8);
        else if (parser.Check(TokenType.Register))
            node = new RegisterNode();
        else
        {
            ErrorHandler.Instance!.Error(parser.Current(), "Expected number or register");
            return;
        }

        node.Populate(parser);
        Children.Add(node);
    }

    public override void Compile(Compiler compiler)
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return "PUSH";
    }

    public override int GetSize()
    {
        return 1 + (Children.Count > 0 ? 1 : 0);
    }
}