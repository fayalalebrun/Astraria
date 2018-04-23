package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.utils.Disposable;

/**
 * Created by Fran on 4/18/2018.
 */
public class GLTexture implements Disposable {
    public final int ID;
    public final FileHandle fileHandle;

    public GLTexture(FileHandle fileHandle) {
        this.fileHandle = fileHandle;

        ID = loadTexture();
    }

    private int loadTexture(){
        Pixmap pixmap = new Pixmap(fileHandle);

        int texture = Gdx.gl.glGenTexture();
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_2D, texture);

        Gdx.gl.glPixelStorei(Gdx.gl.GL_UNPACK_ALIGNMENT, 1);

        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_WRAP_S, Gdx.gl.GL_REPEAT);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_WRAP_T, Gdx.gl.GL_REPEAT);

        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_MIN_FILTER, Gdx.gl.GL_LINEAR);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_MAG_FILTER, Gdx.gl.GL_LINEAR);



        Gdx.gl.glTexImage2D(Gdx.gl.GL_TEXTURE_2D, 0, pixmap.getGLInternalFormat(), pixmap.getWidth(),
                pixmap.getHeight(), 0, pixmap.getGLFormat(), pixmap.getGLType(),
                pixmap.getPixels());

        Gdx.gl.glGenerateMipmap(Gdx.gl.GL_TEXTURE_2D);
        pixmap.dispose();
        return texture;
    }

    @Override
    public String toString() {
        return fileHandle.name();
    }

    @Override
    public boolean equals(Object obj) {
        if(obj instanceof GLTexture){
            return fileHandle.path().equals(((GLTexture)obj).fileHandle.path());
        }
        return false;
    }

    @Override
    public void dispose() {
        Gdx.gl.glDeleteTexture(ID);
    }
}
