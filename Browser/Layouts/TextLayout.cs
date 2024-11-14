using Browser.DrawCommands;
using Browser.Html;
using Browser.Utils;
using SkiaSharp;

namespace Browser.Layouts;

public sealed class TextLayout : Layout
{
    public new readonly TextLayout? Previous;
    public readonly string Word;
    public required SKFont Font;

    public TextLayout(HtmlNode node, string word, Layout? parent = null,
        TextLayout? previous = null)
    {
        Node = node;
        Word = word;
        Parent = parent;
        Previous = previous;
    }

    public override void CalculateLayout()
    {
        var weight = Node.Styles["font-weight"];
        var style = Node.Styles["font-style"];
        var size = (float)(Convert.ToDouble(Node.Styles["font-size"][..^2]) *
                           0.75);
        Font = FontUtils.GetFont(size, weight, style);
        Width = Font.MeasureText(Word);
        if (Previous != null)
        {
            var spaceWidth = Previous.Font.MeasureText(" ");
            X = Previous.X + spaceWidth + Previous.Width;
        }
        else
        {
            X = Parent!.X;
        }

        Height = FontUtils.GetLineHeight(Font);
    }

    public override List<DrawCommand> Paint()
    {
        var drawCommands = new List<DrawCommand>();
        var color = Node.Styles["color"];
        drawCommands.Add(new DrawText(X, Y, Word, Font, color));
        return drawCommands;
    }

    public override bool ShouldPaint()
    {
        return true;
    }

    public override List<DrawCommand> PaintEffects(
        List<DrawCommand> drawCommands)
    {
        return drawCommands;
    }
}