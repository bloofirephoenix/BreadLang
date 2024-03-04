using BreadLang.Compiling;

namespace BreadLang.Parser.Nodes.Placeholders;

public class NextAddressPlaceholderNode : PlaceholderNode
{
	public override void Compile(Compiler compiler)
	{
		compiler.WriteImmediate16(compiler.Position + 2);
	}
	public override void Populate(Parser parser)
	{
		parser.Advance(); // advance past nextaddr
	}

	public override int GetSize()
	{
		return 2;
	}

	public override string ToString()
	{
		return "AddressPlaceholderNode";
	}
}