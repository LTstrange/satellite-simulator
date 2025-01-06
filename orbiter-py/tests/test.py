import unittest
import orbiter
from math import pi
# python -m unittest discover tests


class TestOrbiter(unittest.TestCase):
    def test_orbiter(self):
        with orbiter.Orbiter() as orb:
            sate = orbiter.Satellite()
            sate.set_apogee_perigee(20000, 7000)

            sate.set_mean_anomaly(0)
            sate.set_id(f"test_satellite_{0}")
            print(sate.elements)
            orb.add_satellite(sate)


if __name__ == '__main__':
    unittest.main()
