from functools import wraps
#from typing import Callable, TypeVar, Any, cast, Protocol
from typing import Callable, Any, cast, Protocol
import time
from timeit import timeit


# 定义一个协议，要求类型必须有 __name__ 属性
class NamedCallable(Protocol):
    __name__: str
    def __call__(self, *args: Any, **kwargs: Any) -> Any: ...

# F = TypeVar('F', bound=NamedCallable)

def debug_log1[F: NamedCallable](tag: str ='[info]') -> Callable[[F], F]:
    def actual_decorator(func: F) -> F:
        @wraps(func)  # ty: ignore[invalid-argument-type]
        def wrapper(*args: Any, **kwargs: Any) -> Any:
            print(f"{tag} {func.__name__} start...")
            result = func(*args, **kwargs)
            print(f"{tag} {func.__name__} done.")
            return result
        return cast(F, wrapper)
    return actual_decorator


def debug_log[F: NamedCallable](func: F) -> F:
    @wraps(func)  # ty: ignore[invalid-argument-type]
    def wrapper(*args: Any, **kwargs: Any) -> Any:
        print(f"[info] {func.__name__} start...")
        result = func(*args, **kwargs)
        print(f"[info] {func.__name__} done.")
        return result
    return cast(F, wrapper)


def measure_time1[F: NamedCallable](runs: int=1) -> Callable[[F], F]:
    def actual_decorator(func: F) -> F:
        @wraps(func)  # ty: ignore[invalid-argument-type]
        def wrapper(*args: Any, **kwargs: Any) -> Any:
            total_time = timeit(lambda: func(*args, **kwargs), number=runs)
            avg_time = total_time / runs
            print(f"[measure_time] {func.__name__} 平均执行时间: {avg_time:.6f}秒 (基于{runs}次运行)")
        return cast(F, wrapper)
    return actual_decorator


def measure_time[F: NamedCallable](func: F) -> F:
    @wraps(func)  # ty: ignore[invalid-argument-type]
    def wrapper(*args: Any, **kwargs: Any) -> Any:
        start_time = time.perf_counter()  # 高精度计时器
        result = func(*args, **kwargs)
        end_time = time.perf_counter()
        elapsed_time = end_time - start_time
        print(f"[measure_time] {func.__name__} 执行时间: {elapsed_time:.6f}秒")
        return result

    return cast(F, wrapper)
