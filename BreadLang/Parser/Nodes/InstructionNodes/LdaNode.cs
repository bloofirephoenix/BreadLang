using BreadLang.Compiling;
using BreadLang.Parser.Nodes.Placeholders;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class LdaNode : Node
{
	public override void Populate(Parser parser)
	{
		if (parser.Check(TokenType.NextAddr))
			PopulateAndAdd(new NextAddressPlaceholderNode(), parser);
		else if (parser.Check(TokenType.Number))
			PopulateAndAdd(new NumberNode(NumberNode.Type.Immediate16), parser);
		else
			ErrorHandler.Instance!.Error(parser.Current(), "Expected number or NextAddr");
	}

	public override void Compile(Compiler compiler)
	{
		compiler.WriteFirstByte(OpCodes.Lda, true, null);
		if (Children[0] is NumberNode)
			compiler.WriteImmediate16(((NumberNode)Children[0]).Value);
		else
			Children[0].Compile(compiler);
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