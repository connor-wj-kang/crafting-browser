using Browser.Utils;
using SkiaSharp;

namespace Browser.DrawCommands;

public sealed class DrawRoundRectangle
    : DrawCommand
{
    private readonly string _color;
    private readonly SKRoundRect _roundRectangle;

    public DrawRoundRectangle(
        SKRect rectangle,
        float radius,
        string color)
    {
        Rectangle = rectangle;
        _roundRectangle = new SKRoundRect(rectangle, radius);
        _color = color;
    }

    public override void Draw(SKCanvas canvas)
    {
        var paint = new SKPaint
        {
            Color = ColorUtils.ParseColor(_color)
        };
        canvas.DrawRoundRect(_roundRectangle, paint);
    }
}