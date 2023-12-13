using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class JmpNode : Node
{
    public override void Populate(Parser parser)
    {
        if (parser.Peek().Type == TokenType.Number)
        {
            var number = new NumberNode(NumberNode.Type.Immediate16);
            number.Populate(parser);
            Children.Add(number);
        }
    }

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return "JMP";
    }
}