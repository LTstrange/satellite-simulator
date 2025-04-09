class SatSimManager:
    """
    Manage connections to the Satellite Simulator.
    provide constellation building api.

    """
    def __new__(host: str, port: int):
        """
        Connect to the Satellite Simulator with the given host and port.

        Args:
            host (str): The host name or IP address of the SatSim server.
            port (int): The port number of the SatSim server.
        """
        ...

    def clean_all():
        """
        Remove all satellites from the SatSim simulation.
        """
        ...

    def create_orbit(
        self,
        apogee: float,
        perigee: float,
        inclination: float,
        longitude_of_ascending_node: float,
        argument_of_periapsis: float,
        mean_anomaly: float,
    ):
        """
        Create a new orbit.

        Args:
            apogee: The apogee distance in kilometers.
            perigee : The perigee distance in kilometers.
            inclination: The inclination in radians.
            longitude_of_ascending_node: The longitude of ascending node in radians.
            argument_of_periapsis: The argument of periapsis in radians.
        """
        ...
