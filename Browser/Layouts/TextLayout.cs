using Browser.DrawCommands;
using Browser.Html;
using Browser.Utils;

namespace Browser.Layouts;

public sealed class TextLayout(
    HtmlNode node,
    string word,
    Layout? parent = null,
    Layout? previous = null)
    : Layout(node, parent, previous)
{
    public override void CalculateLayout()
    {
        var weight = Node.Styles["font-weight"];
        var style = Node.Styles["font-style"];
        var size = FontUtils.ParseFontSize(Node.Styles["font-size"][..^2]);
        Font = FontUtils.GetFont(size, weight, style);
        Width = Font.MeasureText(word);
        if (Previous != null)
        {
            var spaceWidth = Previous.Font!.MeasureText(" ");
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
        drawCommands.Add(new DrawText(X, Y, word, Font!, color));
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