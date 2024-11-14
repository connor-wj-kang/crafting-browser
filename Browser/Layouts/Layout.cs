using Browser.DrawCommands;
using Browser.Html;
using SkiaSharp;

namespace Browser.Layouts;

public abstract class Layout(
    HtmlNode node,
    Layout? parent = null,
    Layout? previous = null)
{
    public readonly List<Layout> Children = [];
    protected readonly HtmlNode Node = node;
    protected readonly Layout? Parent = parent;
    protected readonly Layout? Previous = previous;
    public SKFont? Font = null;
    public float Height = 0;
    public float Width = 0;
    public float X = 0;
    public float Y = 0;

    public static void PaintTree(Layout layout,
        List<DrawCommand> drawCommands)
    {
        var result = layout.Paint();
        drawCommands = drawCommands.Concat(result).ToList();
        drawCommands.ForEach(cmd => Console.WriteLine());
        foreach (var child in layout.Children) PaintTree(child, drawCommands);
    }

    public abstract void CalculateLayout();
    public abstract List<DrawCommand> Paint();
    public abstract bool ShouldPaint();

    public abstract List<DrawCommand> PaintEffects(
        List<DrawCommand> drawCommands);
}