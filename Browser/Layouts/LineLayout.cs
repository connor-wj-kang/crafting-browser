using Browser.DrawCommands;
using Browser.Html;

namespace Browser.Layouts;

public sealed class LineLayout : Layout
{
    public LineLayout(HtmlNode node, Layout? parent = null,
        Layout? previous = null)
    {
        Node = node;
        Parent = parent;
        Previous = previous;
    }

    public override void CalculateLayout()
    {
        Width = Parent!.Width;
        X = Parent.X;
        if (Previous != null)
            Y = Previous.Y + Previous.Height;
        else
            Y = Parent.Y;
        Children.ForEach(word => word.CalculateLayout());
        if (Children.Count == 0)
        {
            Height = 0;
            return;
        }

        var maxAscent =
            Children.Select(word =>
                {
                    return word switch
                    {
                        TextLayout textLayout => textLayout.Font.Metrics.Ascent,
                        InputLayout inputLayout => inputLayout.Font.Metrics
                            .Ascent,
                        _ => 0
                    };
                })
                .Max();
        var maxDescent =
            Children.Select(word =>
                {
                    return word switch
                    {
                        TextLayout textLayout => textLayout.Font.Metrics.Descent,
                        InputLayout inputLayout => inputLayout.Font.Metrics.Descent,
                        _ => 0
                    };
                })
                .Max();
        var baseline = Y + 1.25f * maxAscent;
        Children.ForEach(word =>
            word.Y = baseline + word.Font.Metrics.Ascent);
        Height = 1.25f * (maxAscent + maxDescent);
    }

    public override List<DrawCommand> Paint()
    {
        return [];
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