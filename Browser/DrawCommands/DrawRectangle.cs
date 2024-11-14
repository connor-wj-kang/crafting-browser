using Browser.Utils;
using SkiaSharp;

namespace Browser.DrawCommands;

public sealed class DrawRectangle : DrawCommand
{
    private readonly string _color;

    public DrawRectangle(SKRect rectangle, string color)
    {
        Rectangle = rectangle;
        _color = color;
    }

    public override void Draw(SKCanvas canvas)
    {
        var paint = new SKPaint
        {
            Color = ColorUtils.ParseColor(_color)
        };
        canvas.DrawRect(Rectangle, paint);
    }
}