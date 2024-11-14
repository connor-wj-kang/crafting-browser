using Browser.DrawCommands;
using Browser.Html;

namespace Browser.Layouts;

public sealed class DocumentLayout(HtmlNode node) : Layout(node)
{
    public override void CalculateLayout()
    {
        var child = new BlockLayout(Node, this);
        Children.Add(child);
        Width = 800 - 2 * 16;
        X = 16;
        Y = 18;
        child.CalculateLayout();
        Height = child.Height;
    }

    public override List<DrawCommand> Paint()
    {
        return [];
    }

    public override bool ShouldPaint()
    {
        return true;
    }

    public override List<DrawCommand> PaintEffects(List<DrawCommand> drawCommands)
    {
        return drawCommands;
    }
}