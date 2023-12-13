using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class LwNode : Node
{
    public override void Populate(Parser parser)
    {
        var registerNode = new RegisterNode();
        registerNode.Populate(parser);
        Children.Add(registerNode);

        if (parser.Peek().Type == TokenType.Comma)
        {
            // there is an immediate value
            parser.Advance(); // comma
            var numberNode = new NumberNode(NumberNode.Type.Immediate16);
            numberNode.Populate(parser);
            Children.Add(numberNode);
        }

    }

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return "LW";
    }
}