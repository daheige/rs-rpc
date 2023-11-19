<?php

// autoload_static.php @generated by Composer

namespace Composer\Autoload;

class ComposerStaticInitf0e102420cbea6ed25047df55dfd9dc8
{
    public static $prefixLengthsPsr4 = array (
        'G' => 
        array (
            'Grpc\\' => 5,
        ),
        'A' => 
        array (
            'App\\Grpc\\Hello\\' => 15,
            'App\\Grpc\\GPBMetadata\\' => 21,
        ),
    );

    public static $prefixDirsPsr4 = array (
        'Grpc\\' => 
        array (
            0 => __DIR__ . '/..' . '/grpc/grpc/src/lib',
        ),
        'App\\Grpc\\Hello\\' => 
        array (
            0 => __DIR__ . '/../..' . '/App/Grpc/Hello',
        ),
        'App\\Grpc\\GPBMetadata\\' => 
        array (
            0 => __DIR__ . '/../..' . '/App/Grpc/GPBMetadata',
        ),
    );

    public static $classMap = array (
        'Composer\\InstalledVersions' => __DIR__ . '/..' . '/composer/InstalledVersions.php',
    );

    public static function getInitializer(ClassLoader $loader)
    {
        return \Closure::bind(function () use ($loader) {
            $loader->prefixLengthsPsr4 = ComposerStaticInitf0e102420cbea6ed25047df55dfd9dc8::$prefixLengthsPsr4;
            $loader->prefixDirsPsr4 = ComposerStaticInitf0e102420cbea6ed25047df55dfd9dc8::$prefixDirsPsr4;
            $loader->classMap = ComposerStaticInitf0e102420cbea6ed25047df55dfd9dc8::$classMap;

        }, null, ClassLoader::class);
    }
}