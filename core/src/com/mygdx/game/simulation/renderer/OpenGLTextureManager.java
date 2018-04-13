package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;

import java.nio.IntBuffer;
import java.util.HashMap;
import java.util.Map;

/**
 * Created by fraayala19 on 3/15/18.
 */
public class OpenGLTextureManager implements Disposable{
    private Map<String, Integer> textures;

    public OpenGLTextureManager() {
        textures = new HashMap<String, Integer>();
    }

    public int addTexture(String path){
        if(!textures.containsKey(path)){
            textures.put(path,loadTexture(new FileHandle(path)));
        }
        return textures.get(path);
    }

    private int loadTexture(FileHandle handle){

        Pixmap pixmap = new Pixmap(handle);

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

    public int loadCubeMap(FileHandle[] handles){
        Pixmap[] pixmaps = new Pixmap[handles.length];

        for(int i = 0; i < pixmaps.length; i++){
            pixmaps[i] = new Pixmap(handles[i]);
        }

        int texture = Gdx.gl.glGenTexture();
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_CUBE_MAP, texture);

        for(int i = 0; i < pixmaps.length; i++){
            Gdx.gl.glTexImage2D(Gdx.gl.GL_TEXTURE_CUBE_MAP_POSITIVE_X + i, 0,
                    pixmaps[i].getGLInternalFormat(), pixmaps[i].getWidth(),
                    pixmaps[i].getHeight(), 0, pixmaps[i].getGLFormat(), pixmaps[i].getGLType(),
                    pixmaps[i].getPixels());
        }

        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_CUBE_MAP, Gdx.gl.GL_TEXTURE_MIN_FILTER, Gdx.gl.GL_LINEAR);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_CUBE_MAP, Gdx.gl.GL_TEXTURE_MAG_FILTER, Gdx.gl.GL_LINEAR);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_CUBE_MAP, Gdx.gl.GL_TEXTURE_WRAP_S, Gdx.gl.GL_CLAMP_TO_EDGE);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_CUBE_MAP, Gdx.gl.GL_TEXTURE_WRAP_T, Gdx.gl.GL_CLAMP_TO_EDGE);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_CUBE_MAP, Gdx.gl30.GL_TEXTURE_WRAP_R, Gdx.gl.GL_CLAMP_TO_EDGE);

        for(int i = 0; i < pixmaps.length; i++){
            pixmaps[i].dispose();
        }

        return texture;

    }


    @Override
    public void dispose() {
        for(Integer i : textures.values()){
            Gdx.gl.glDeleteTexture(i);
        }
    }
}
