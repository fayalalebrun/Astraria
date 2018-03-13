package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;

import java.io.File;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.IntBuffer;

import static com.badlogic.gdx.graphics.GL20.GL_VERTEX_SHADER;

/**
 * Created by Fran on 3/12/2018.
 */
public class Shader {
    public int ID;

    public Shader(FileHandle vertexPath, FileHandle fragmentPath) {
        String vertexCode = vertexPath.readString();
        String fragmentCode = fragmentPath.readString();
        int vertex, fragment;

        vertex = Gdx.gl.glCreateShader(Gdx.gl.GL_VERTEX_SHADER);
        Gdx.gl.glShaderSource(vertex, vertexCode);
        Gdx.gl.glCompileShader(vertex);
        checkCompileErrors(vertex, "VERTEX");

        fragment = Gdx.gl.glCreateShader(Gdx.gl.GL_FRAGMENT_SHADER);
        Gdx.gl.glShaderSource(fragment, fragmentCode);
        Gdx.gl.glCompileShader(fragment);
        checkCompileErrors(fragment, "FRAGMENT");

        ID = Gdx.gl.glCreateProgram();
        Gdx.gl.glAttachShader(ID, vertex);
        Gdx.gl.glAttachShader(ID, fragment);
        Gdx.gl.glLinkProgram(ID);
        checkCompileErrors(ID, "PROGRAM");

        Gdx.gl.glDeleteShader(vertex);
        Gdx.gl.glDeleteShader(fragment);
    }

    private void checkCompileErrors(int shader, String type){

        ByteBuffer temp = ByteBuffer.allocateDirect(4);
        temp.order(ByteOrder.nativeOrder());
        IntBuffer success = temp.asIntBuffer();

        if(!type.equals("PROGRAM")){
            Gdx.gl.glGetShaderiv(shader, Gdx.gl.GL_COMPILE_STATUS, success);
            if(success.get(0)==0){
                System.out.println("ERROR::SHADER_COMPILATION_ERROR of type: "+type+"\n"+
                        Gdx.gl.glGetShaderInfoLog(shader));
            }
        } else {
            Gdx.gl.glGetProgramiv(shader, Gdx.gl.GL_LINK_STATUS, success);
            if(success.get(0)==0){
                System.out.println("ERROR::PROGRAM_LINKING_ERROR of type: "+type+"\n"+
                        Gdx.gl.glGetProgramInfoLog(shader));
            }
        }
    }
}
