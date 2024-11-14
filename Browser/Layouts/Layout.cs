using Browser.DrawCommands;
using Browser.Html;
using SkiaSharp;

namespace Browser.Layouts;

public abstract class Layout
{
    public readonly List<Layout> Children = [];
    public float Height = 0;
    protected readonly HtmlNode Node;
    protected readonly Layout? Parent = null;
    protected readonly Layout? Previous = null;
    public float Width = 0;
    public float X = 0;
    public float Y = 0;
    public SKFont? Font = null;

    protected Layout(HtmlNode node, Layout? parent = null, Layout? previous = null)
    {
        Node = node;
        Parent = parent;
        Previous = previous;
    }

    public abstract void CalculateLayout();
    public abstract List<DrawCommand> Paint();
    public abstract bool ShouldPaint();
    public abstract List<DrawCommand> PaintEffects(
        List<DrawCommand> drawCommands);
}