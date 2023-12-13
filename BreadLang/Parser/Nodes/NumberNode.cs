using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes;

public class NumberNode(NumberNode.Type type) : Node
{
    public enum Type
    {
        Immediate8, Immediate16
    }

    public int Value { get; set; }
    public override void Populate(Parser parser)
    {
        var token = parser.Expect(TokenType.Number);
        var numberString = token.Lexeme.Replace("_", "");
        
        // hexadecimal processing
        try
        {
            if (numberString.ToLower().StartsWith("0x"))
            {
                Value = int.Parse(numberString.Substring(2), System.Globalization.NumberStyles.HexNumber);
            } else if (numberString.ToLower().StartsWith("0b"))
            {
                Value = int.Parse(numberString.Substring(2), System.Globalization.NumberStyles.BinaryNumber);
            }
            else
            {
                Value = int.Parse(numberString);
            }
        }
        catch (FormatException e)
        {
            ErrorHandler.Instance!.Error(token, "Invalid number");
            return;
        }
        
        // check if the number fits
        switch (type)
        {
            case Type.Immediate8:
                if ((Value & 0xFFFFFF00) == 0)
                    return;
                break;
            case Type.Immediate16:
                if ((Value & 0xFFFF0000) == 0)
                    return;
                break;
        }
        ErrorHandler.Instance!.Error(token, "Number is too large");
    }

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return $"Number({Value})";
    }
}