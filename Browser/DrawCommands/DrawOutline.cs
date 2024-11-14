using Browser.Utils;
using SkiaSharp;

namespace Browser.DrawCommands;

public sealed class DrawOutline : DrawCommand
{
    private readonly string _color;
    private readonly float _thickness;

    public DrawOutline(SKRect rectangle, string color, float thickness)
    {
        Rectangle = rectangle;
        _color = color;
        _thickness = thickness;
    }

    public override void Draw(SKCanvas canvas)
    {
        var paint = new SKPaint
        {
            Color = ColorUtils.ParseColor(_color),
            StrokeWidth = _thickness,
            Style = SKPaintStyle.Stroke
        };
        canvas.DrawRect(Rectangle, paint);
    }
}