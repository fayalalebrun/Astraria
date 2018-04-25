package com.mygdx.game.desktop;

import com.badlogic.gdx.Files;
import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.backends.lwjgl.LwjglApplication;
import com.badlogic.gdx.backends.lwjgl.LwjglApplicationConfiguration;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.glutils.ShaderProgram;
import com.mygdx.game.Boot;

import javax.swing.*;

public class DesktopLauncher {
	public static void main (String[] arg) {
		LwjglApplicationConfiguration config = new LwjglApplicationConfiguration();
		config.useGL30 = true;
		config.samples=7;

		config.addIcon("icons/astraria256.png", Files.FileType.Internal);
		config.addIcon("icons/astraria128.png", Files.FileType.Internal);
		config.addIcon("icons/astraria32.png", Files.FileType.Internal);
		config.addIcon("icons/astraria16.png", Files.FileType.Internal);
		ShaderProgram.prependVertexCode = "#version 140\n#define varying out\n#define attribute in\n";
		ShaderProgram.prependFragmentCode = "#version 140\n#define varying in\n#define texture2D texture\n#define gl_FragColor fragColor\nout vec4 fragColor;\n";
		new LwjglApplication(new Boot(arg), config);
	}
}
