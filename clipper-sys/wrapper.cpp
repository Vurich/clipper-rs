#include "./wrapper.hpp"
#include "./clipper/cpp/clipper.cpp"

extern "C" {
  //****************** PATH
  path path_new(int numPoints)
  {
    if(numPoints > 0)
      return new Path(numPoints);
    else
      return new Path();
  }

  void path_clear(path poly)
  {
    ((Path *) poly)->clear();
  }

  int path_size(path poly)
  {
    return ((Path *) poly)->size();
  }

  void path_addPoint(path poly, long64 x, long64 y)
  {
    IntPoint pt(x, y);
    ((Path *)poly)->push_back(pt);
  }

  long64 path_getPointX(path poly, int i)
  {
    return (*(Path *)poly)[i].X;
  }

  long64 path_getPointY(path poly, int i)
  {
    return (*(Path *)poly)[i].Y;
  }

  void path_free(path poly)
  {
    delete (Path *) poly;
  }

  double path_getArea(path poly)
  {
    return Area(*((Path *) poly));
  }

  //****************** PATHS
  paths paths_new(int numPolys)
  {
    if(numPolys > 0)
      return new Paths(numPolys);
    else
      return new Paths();
  }

  void paths_clear(paths poly)
  {
    ((Paths *) poly)->clear();
  }

  int paths_size(paths poly)
  {
    return ((Paths *) poly)->size();
  }

  path paths_getPath(paths polys, int i)
  {
    return &((Paths *)polys)->at(i);
  }

  void paths_addPath(paths polys, path poly)
  {
    ((Paths *)polys)->push_back(*((Path *)poly));
  }

  void paths_free(paths poly)
  {
    delete (Paths *) poly;
  }

  //****************** CLIPPER
  clipper clipper_new()
  {
    return new Clipper();
  }

  void clipper_addPath(clipper c, path poly, PolyType ptype)
  {
    ((Clipper *) c)->AddPath(*((Path *) poly), ptype, true);
  }

  void clipper_addPaths(clipper c, paths poly, PolyType ptype)
  {
    ((Clipper *) c)->AddPaths(*((Paths *) poly), ptype, true);
  }

  void clipper_execute(clipper c, ClipType ctype, paths soln)
  {
    ((Clipper *) c)->Execute(ctype, *((Paths *) soln));
  }

  void clipper_free(clipper c)
  {
    delete ((Clipper *) c);
  }

  //****************** CLIPPEROFFSET
  clipper clipperoffset_new()
  {
    return new ClipperOffset();
  }

  void clipperoffset_setMiterLimit(clipperoffset c, double limit)
  {
    ((ClipperOffset *) c)->MiterLimit = limit;
  }

  void clipperoffset_setArcTolerance(clipperoffset c, double tolerance)
  {
    ((ClipperOffset *) c)->ArcTolerance = tolerance;
  }

  void clipperoffset_addPath(clipperoffset c, path poly, JoinType jtype, EndType etype)
  {
    ((ClipperOffset *) c)->AddPath(*((Path *) poly), jtype, etype);
  }

  void clipperoffset_addPaths(clipperoffset c, paths poly, JoinType jtype, EndType etype)
  {
    ((ClipperOffset *) c)->AddPaths(*((Paths *) poly), jtype, etype);
  }

  void clipperoffset_execute(clipperoffset c, paths solution, double delta)
  {
    ((ClipperOffset *) c)->Execute(*((Paths *) solution), delta);
  }

  void clipperoffset_free(clipperoffset c)
  {
    delete ((ClipperOffset *) c);
  }
}
