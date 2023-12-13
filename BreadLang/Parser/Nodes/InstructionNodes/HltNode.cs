namespace BreadLang.Parser.Nodes.InstructionNodes;

public class HltNode : Node
{
    public override void Populate(Parser parser)
    {
        // do nothing
    }

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        throw new NotImplementedException();
    }
}