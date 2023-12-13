namespace BreadLang.Parser.Nodes.InstructionNodes;

public class NopNode : Node
{
    public override void Populate(Parser parser)
    {
        // do nothing
    }

    public override byte[] Compile()
    {
        return new byte[0b0000];
    }

    public override string ToString()
    {
        return "NOP";
    }
}