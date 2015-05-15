initSidebarItems({"enum":[["BasicForm","The basic variants a Form can consist of."],["FillStyle",""],["LineCap",""],["LineJoin",""],["ShapeStyle","Whether a shape is outlined or filled."]],"fn":[["circle","A circle with a given radius."],["collage","A collage is a collection of 2D forms. There are no strict positioning relationships between forms, so you are free to do all kinds of 2D graphics."],["dashed","Create a dashed line style with a given color. Dashing equals `[8, 4]`."],["dotted","Create a dotted line style with a given color. Dashing equals `[3, 3]`."],["draw_form",""],["group","Flatten many forms into a single `Form`. This lets you move and rotate them as a single unit, making it possible to build small, modular components."],["group_transform","Flatten many forms into a single `Form` and then apply a matrix transformation."],["line","Create a line with a given line style."],["ngon","A regular polygon with N sides. The first argument specifies the number of sides and the second is the radius. So to create a pentagon with radius 30, you would say `ngon(5, 30.0)`"],["oval","An oval with a given width and height."],["point_path","Create a PointPath that follows a sequence of points."],["polygon","Create an arbitrary polygon by specifying its corners in order. `polygon` will automatically close all shapes, so the given list of points does not need to start and end with the same position."],["rect","A rectangle with a given width and height."],["segment","Create a PointPath along a given line segment."],["solid","Create a solid line style with a given color."],["sprite","Create a sprite from a sprite sheet. It cuts out a rectangle at a given position."],["square","A square with a given edge length."],["text","Create some text. Details like size and color are part of the `Text` value itself, so you can mix colors and sizes and fonts easily."],["to_form","Turn any `Element` into a `Form`. This lets you use text, gifs, and video in your collage. This means you can move, rotate, and scale an `Element` however you want."],["traced","Trace a path with a given line style."]],"struct":[["Form","A general, freeform 2D graphics structure."],["LineStyle",""],["PointPath","A path described by a sequence of points."],["Shape","A shape described by its edges."]]});