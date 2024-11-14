using Browser.DrawCommands;
using Browser.Html;

namespace Browser.Layouts;

public abstract class Layout
{
    public readonly List<Layout> Children = [];
    public float Height = 0;
    public HtmlNode Node;
    public Layout? Parent = null;
    public Layout? Previous = null;
    public float Width = 0;
    public float X = 0;
    public float Y = 0;

    public abstract void CalculateLayout();
    public abstract List<DrawCommand> Paint();
    public abstract bool ShouldPaint();

    public abstract List<DrawCommand> PaintEffects(
        List<DrawCommand> drawCommands);
}