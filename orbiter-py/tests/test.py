import unittest
import orbiter
from math import pi
# python -m unittest discover tests


class TestOrbiter(unittest.TestCase):
    def test_orbiter(self):
        with orbiter.Orbiter() as orb:
            mean_motion = 15 * 2 * pi / 86400
            eccentricity = 0
            inclination = 0
            longitude_of_ascending_node = 0
            argument_of_periapsis = 0
            for i in range(100):
                mean_anomaly = i * 2 * pi / 100
                orb.add_satellite(f"test_satellite_{i}", mean_motion,
                                  eccentricity, inclination, longitude_of_ascending_node, argument_of_periapsis, mean_anomaly)
            # mean_anomaly = 0

            # orb.add_satellite("test_satellite", mean_motion,
            #                   eccentricity, inclination, longitude_of_ascending_node, argument_of_periapsis, mean_anomaly)


if __name__ == '__main__':
    unittest.main()
