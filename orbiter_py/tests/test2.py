from orbiter_py import SatSimManager
from math import pi


def main():
    m = SatSimManager("127.0.0.1", 8080)

    orbit = m.create_orbit(
        6371 + 550, 6371 + 550, pi / 3, 2 * pi / 10, 2 * pi / 10, 2 * pi / 10
    )


if __name__ == "__main__":
    main()
