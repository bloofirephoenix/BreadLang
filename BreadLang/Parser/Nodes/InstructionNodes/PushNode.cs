using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class PushNode : Node
{
    public override void Populate(Parser parser)
    {
        Node node;
        if (parser.Peek().Type == TokenType.Number)
        {
            node = new NumberNode(NumberNode.Type.Immediate8);
        }
        else
        {
            node = new RegisterNode();
        }
        node.Populate(parser);
        Children.Add(node);
    }

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return "PUSH";
    }
}