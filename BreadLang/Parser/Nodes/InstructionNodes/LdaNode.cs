namespace BreadLang.Parser.Nodes.InstructionNodes;

public class LdaNode : Node
{
    public override void Populate(Parser parser)
    {
        var number = new NumberNode(NumberNode.Type.Immediate16);
        number.Populate(parser);
        Children.Add(number);
    }

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return "LDA";
    }
}