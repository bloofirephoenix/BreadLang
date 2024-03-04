namespace BreadLang.Compiling;

public class Compiler
{
	public Dictionary<string, byte[]> Placeholders = new();
	private BinaryWriter _writer;
	public int Position { get; private set; } = 0;

	public Compiler(MemoryStream stream)
	{
		_writer = new(stream);
	}

	public RegisterSelect GetRegister(string name)
	{
		if (Enum.TryParse(name, out RegisterSelect register))
		{
			return register;
		}

		throw new Exception($"Bad register name {name}");
	}

	public void Write(byte value)
	{
		Position ++;
		_writer.Write(value);
	}

	public void Write(byte[] bytes)
	{
		Position += bytes.Length;
		_writer.Write(bytes);
	}

	public void WriteFirstByte(OpCodes instruction, bool immediate, RegisterSelect? register)
	{
		var opCode = (int) instruction;

		var b = 0b00000000;

		b |= opCode << 4;
		if (immediate)
			b |= 0b00001000;

		if (register != null)
		{
			b |= (int)register;
		}

		Write((byte) b);
	}

	public byte[] GetImmediate16(int i)
	{
		return new byte[] { (byte)(i >> 8), (byte) i };
	}

	public void WriteImmediate16(int i)
	{
		Write(GetImmediate16(i));
	}

	public void WriteImmediate8(int i)
	{
		Write((byte) i);
	}

	public void WriteTwoBytes(OpCodes instruction, bool immediate, RegisterSelect register1, RegisterSelect register2)
	{
		int registerCode = (int) register2;
		int b = 0b00000000;
		b |= registerCode << 5;

		WriteFirstByte(instruction, immediate, register1);
		Write((byte) b);
	}
}