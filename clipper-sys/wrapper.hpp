#include <cstdlib>
#include "./clipper/cpp/clipper.hpp"

using namespace ClipperLib;

extern "C" {
  typedef void * path;
  typedef void * paths;
  typedef void * clipper;
  typedef void * clipperoffset;

  //****************** PATH
  path path_new(int);

  void path_clear(path);

  int path_size(path);

  void path_addPoint(path, long64, long64);

  long64 path_getPointX(path, int);

  long64 path_getPointY(path, int);

  void path_free(path);

  double path_getArea(path);

  //****************** PATHS
  paths paths_new(int);

  void paths_clear(paths);

  int paths_size(paths);

  path paths_getPath(paths, int);

  void paths_addPath(paths, path);

  void paths_free(paths);

  //****************** CLIPPER
  clipper clipper_new();

  void clipper_addPath(clipper, path, PolyType);

  void clipper_addPaths(clipper, paths, PolyType);

  void clipper_execute(clipper, ClipType, paths);

  void clipper_free(clipper);

  //****************** CLIPPEROFFSET
  clipper clipperoffset_new();

  void clipperoffset_setMiterLimit(clipperoffset c, double limit);

  void clipperoffset_setArcTolerance(clipperoffset c, double tolerance);

  void clipperoffset_addPath(clipperoffset c, path poly, JoinType jtype, EndType etype);

  void clipperoffset_addPaths(clipperoffset c, paths poly, JoinType jtype, EndType etype);

  void clipperoffset_execute(clipperoffset c, paths solution, double delta);

  void clipperoffset_free(clipperoffset c);
}
