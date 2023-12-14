using BreadLang.Compiling;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class NopNode : Node
{
    public override void Populate(Parser parser)
    {
        // do nothing
    }

    public override void Compile(Compiler compiler)
    {

    }

    public override int GetSize()
    {
        return 1;
    }

    public override string ToString()
    {
        return "NOP";
    }
}