using BreadLang.Compiling;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class HltNode : Node
{
    public override void Populate(Parser parser)
    {
        // do nothing
    }

    public override void Compile(Compiler compiler)
    {
        compiler.WriteFirstByte(OpCodes.Hlt, false, null);
    }

    public override string ToString()
    {
        throw new NotImplementedException();
    }

    public override int GetSize()
    {
        return 1;
    }
}