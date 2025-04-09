import unittest
import orbiter
from math import pi
# python -m unittest discover tests


class TestOrbiter(unittest.TestCase):
    def test_orbiter(self):
        with orbiter.Orbiter() as orb:
            satellites = []
            sate = orbiter.Satellite()
            sate.set_apogee_perigee(6371 + 550, 6371 + 550)
            sate.set_inclination(pi / 3)
            for j in range(10):
                sate.set_longitude_of_ascending_node(j * 2 * pi / 10)
                for i in range(10):
                    sate.set_mean_anomaly(i * 2 * pi / 10)
                    sate.set_id(f"test_satellite_{0}")
                    satellites.append(sate.build())

            assert satellites[0] != satellites[1]
            orb.add_satellites(satellites)


if __name__ == "__main__":
    unittest.main()
