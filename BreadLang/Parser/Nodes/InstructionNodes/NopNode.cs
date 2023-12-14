using BreadLang.Compiling;
using OpCodes = BreadLang.Compiling.OpCodes;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class NopNode : Node
{
    public override void Populate(Parser parser)
    {
        // do nothing
    }

    public override void Compile(Compiler compiler)
    {
        compiler.WriteFirstByte(OpCodes.Nop, false, null);
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