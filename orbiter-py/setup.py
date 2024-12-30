from setuptools import setup, find_packages

setup(
    name='orbiter-py',
    version='0.1.0',
    description='A satellite simulator python connector',
    author='LTstrange',
    author_email='835422774@qq.com',
    packages=find_packages(),
    install_requires=[
        # List your project dependencies here
        # e.g., 'numpy', 'pandas', etc.
    ],
    classifiers=[
        'Programming Language :: Python :: 3',
        'Operating System :: OS Independent',
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Science/Research',
    ],
    python_requires='>=3.6',
)
