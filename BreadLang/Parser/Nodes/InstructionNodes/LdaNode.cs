using BreadLang.Compiling;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class LdaNode : Node
{
    public override void Populate(Parser parser)
    {
        PopulateAndAdd(new NumberNode(NumberNode.Type.Immediate16), parser);
    }

    public override void Compile(Compiler compiler)
    {
        compiler.WriteFirstByte(OpCodes.Lda, true, null);
        compiler.WriteImmediate16(((NumberNode)Children[0]).Value);
    }

    public override string ToString()
    {
        return "LDA";
    }

    public override int GetSize()
    {
        return 3;
    }
}