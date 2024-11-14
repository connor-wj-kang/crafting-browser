using Browser.DrawCommands;
using Browser.Html;
using Browser.Utils;
using SkiaSharp;

namespace Browser.Layouts;

public class InputLayout(
    HtmlNode node,
    Layout? parent = null,
    Layout? previous = null)
    : Layout(node, parent, previous)
{
    public const float InputWidthPx = 200;

    public override void CalculateLayout()
    {
        var weight = Node.Styles["font-weight"];
        var style = Node.Styles["font-style"];
        var size = FontUtils.ParseFontSize(Node.Styles["font-size"][..^2]);
        Font = FontUtils.GetFont(size, weight, style);
        Width = InputWidthPx;
        Height = FontUtils.GetLineHeight(Font);
        if (Previous != null)
        {
            var spaceWidth = Previous.Font!.MeasureText(" ");
            X = Previous.X + spaceWidth + Previous.Width;
        }
        else 
        {
            X = Parent!.X;
        }
    }

    private SKRect GetInputLayoutRectangle()
    {
        return new SKRect(X, Y, X + Width, Y + Height);
    }

    public override List<DrawCommand> Paint()
    {
        var drawCommands = new List<DrawCommand>();
        var bgColor =
            Node.Styles.GetValueOrDefault("background-color", "transparent");
        if (bgColor != "transparent")
        {
            var radius = (float)Convert.ToDouble(
                Node.Styles.GetValueOrDefault("border-radius", "0px")[..^2]);
            drawCommands.Add(new DrawRoundRectangle(GetInputLayoutRectangle(), radius,
                bgColor));
        }

        var text = "";
        switch (Node)
        {
            case HtmlElement { TagName: "input" } htmlElement:
                text = htmlElement.Attributes.GetValueOrDefault("value", "");
                break;
            case HtmlElement { TagName: "button" } when Node.Children is
                [HtmlText htmlText]:
                text = htmlText.Text;
                break;
            case HtmlElement { TagName: "button" }:
                Console.WriteLine("Ignoring HTML Contents inside button");
                text = "";
                break;
        }

        var color = Node.Styles["color"];
        drawCommands.Add(new DrawText(X, Y, text, Font!, color));
        if (!Node.IsFocused) return drawCommands;
        var cursorX = X + Font!.MeasureText(text);
        drawCommands.Add(new DrawLine(cursorX, Y, cursorX, Y + Height,
            "black", 1));
        return drawCommands;
    }

    public override bool ShouldPaint()
    {
        return true;
    }

    public override List<DrawCommand> PaintEffects(
        List<DrawCommand> drawCommands)
    {
        return Blend.PaintVisualEffects(Node, drawCommands,
            GetInputLayoutRectangle());
    }
}