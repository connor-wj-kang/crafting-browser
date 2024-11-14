using Browser.Html;
using SkiaSharp;

namespace Browser.DrawCommands;

public sealed class Blend : DrawCommand
{
    public readonly string BlendMode;
    public readonly List<DrawCommand> Children;
    public readonly float Opacity;
    public readonly bool ShouldSave;

    public Blend(float opacity, string blendMode, List<DrawCommand> children)
    {
        Opacity = opacity;
        BlendMode = blendMode;
        ShouldSave = blendMode != "" || Opacity < 1;
        Children = children;
        Rectangle = SKRect.Empty;
        Children.ForEach(command => Rectangle.Union(command.Rectangle));
    }

    public override void Draw(SKCanvas canvas)
    {
        var paint = new SKPaint
        {
            BlendMode = ParseBlendMode(BlendMode)
        };
        if (ShouldSave) canvas.SaveLayer(paint);
        Children.ForEach(command => command.Draw(canvas));
        if (ShouldSave) canvas.Restore();
    }

    public static SKBlendMode ParseBlendMode(string blendMode)
    {
        return blendMode switch
        {
            "multiply" => SKBlendMode.Multiply,
            "difference" => SKBlendMode.Difference,
            "destination-in" => SKBlendMode.DstIn,
            "source-over" => SKBlendMode.SrcOver,
            _ => SKBlendMode.SrcOver
        };
    }

    public static List<DrawCommand> PaintVisualEffects(HtmlNode node,
        List<DrawCommand> drawCommands, SKRect rectangle)
    {
        var opacity =
            (float)Convert.ToDouble(
                node.Styles.GetValueOrDefault("opacity", "1.0"));
        var blendMode = node.Styles.GetValueOrDefault("mix-blend-mode", "");
        if (node.Styles.GetValueOrDefault("overflow", "visible") != "clip")
            return [new Blend(opacity, blendMode, drawCommands)];
        var borderRadius =
            (float)Convert.ToDouble(
                node.Styles.GetValueOrDefault("border-radius",
                    "0px")[..^2]);
        if (blendMode == "") blendMode = "source-over";
        drawCommands.Add(new Blend(1.0f, "destination-in",
            [new DrawRoundRectangle(rectangle, borderRadius, "white")]));

        return [new Blend(opacity, blendMode, drawCommands)];
    }
}