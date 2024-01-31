package controllers

import (
	"fmt"
	"github.com/elabosak233/pgshub/services"
	"github.com/elabosak233/pgshub/utils/convertor"
	"github.com/gabriel-vasile/mimetype"
	"github.com/gin-gonic/gin"
	"mime/multipart"
	"net/http"
	"os"
)

var (
	assetsPath string = "./uploads"
)

type AssetController interface {
	GetUserAvatarList(ctx *gin.Context)
	SetUserAvatarByUserId(ctx *gin.Context)     // 设置用户头像
	DeleteUserAvatarByUserId(ctx *gin.Context)  // 删除用户头像
	GetUserAvatarByUserId(ctx *gin.Context)     // 获取用户头像
	GetUserAvatarInfoByUserId(ctx *gin.Context) // 获取用户头像信息
	GetTeamAvatarList(ctx *gin.Context)
	SetTeamAvatarByTeamId(ctx *gin.Context)     // 设置团队头像
	DeleteTeamAvatarByTeamId(ctx *gin.Context)  // 删除团队头像
	GetTeamAvatarByTeamId(ctx *gin.Context)     // 获取团队头像
	GetTeamAvatarInfoByTeamId(ctx *gin.Context) // 获取团队头像信息
	GetGameCoverByGameId(ctx *gin.Context)      // 获取比赛封面
	SetGameCoverByGameId(ctx *gin.Context)      // 设置比赛封面
	FindGameWriteUpByTeamId(ctx *gin.Context)
	SetChallengeAttachmentByChallengeId(ctx *gin.Context)     // 设置题目附件
	DeleteChallengeAttachmentByChallengeId(ctx *gin.Context)  // 删除题目附件
	GetChallengeAttachmentByChallengeId(ctx *gin.Context)     // 获取题目附件
	GetChallengeAttachmentInfoByChallengeId(ctx *gin.Context) // 获取题目附件信息
}

type AssetControllerImpl struct {
	AssetService services.AssetService
}

func NewAssetControllerImpl(appService *services.Services) AssetController {
	return &AssetControllerImpl{
		AssetService: appService.AssetService,
	}
}

// GetUserAvatarList
// @Summary 获取拥有头像的用户列表
// @Description 获取拥有头像的用户列表
// @Tags 资源
// @Accept json
// @Produce json
// @Router /api/assets/users/avatar/ [get]
func (c *AssetControllerImpl) GetUserAvatarList(ctx *gin.Context) {
	res, _ := c.AssetService.GetUserAvatarList()
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": res,
	})
}

// GetUserAvatarByUserId
// @Summary 通过用户 Id 获取用户头像
// @Description 通过用户 Id 获取用户头像
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "用户 Id"
// @Router /api/assets/users/avatar/{id} [get]
func (c *AssetControllerImpl) GetUserAvatarByUserId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("%s/users/avatar/%s", assetsPath, id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.File(path)
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
	}
}

// GetUserAvatarInfoByUserId
// @Summary 通过用户 Id 获得用户头像信息
// @Description 通过用户 Id 获得用户头像信息
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "用户 Id"
// @Router /api/assets/users/avatar/{id}/info [get]
func (c *AssetControllerImpl) GetUserAvatarInfoByUserId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("%s/users/avatar/%s", assetsPath, id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
	}
}

// SetUserAvatarByUserId
// @Summary 通过用户 Id 设置用户头像
// @Description 通过用户 Id 设置用户头像
// @Tags 资源
// @Accept multipart/form-data
// @Param id path string true "用户 Id"
// @Param avatar formData file true "头像文件"
// @Router /api/assets/users/avatar/{id} [post]
func (c *AssetControllerImpl) SetUserAvatarByUserId(ctx *gin.Context) {
	id := ctx.Param("id")
	file, err := ctx.FormFile("avatar")
	if err != nil {
		ctx.Status(http.StatusBadRequest)
		return
	}
	mime, err := c.detectContentType(file)
	if !mime.Is("image/jpeg") && !mime.Is("image/png") && !mime.Is("image/gif") {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "格式不被允许",
		})
		return
	}
	err = ctx.SaveUploadedFile(file, fmt.Sprintf("%s/users/avatar/%s", assetsPath, id))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// DeleteUserAvatarByUserId
// @Summary 通过用户 Id 删除用户头像
// @Description 通过用户 Id 删除用户头像
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "用户 Id"
// @Router /api/assets/users/avatar/{id} [delete]
func (c *AssetControllerImpl) DeleteUserAvatarByUserId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("%s/users/avatar/%s", assetsPath, id)
	_, err := os.Stat(path)
	if err == nil {
		err = os.Remove(path)
		if err != nil {
			ctx.JSON(http.StatusOK, gin.H{
				"code": http.StatusInternalServerError,
				"msg":  err.Error(),
			})
			return
		}
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
	}
}

// GetTeamAvatarList
// @Summary 获取拥有头像的团队列表
// @Description 获取拥有头像的团队列表
// @Tags 资源
// @Accept json
// @Produce json
// @Router /api/assets/teams/avatar/ [get]
func (c *AssetControllerImpl) GetTeamAvatarList(ctx *gin.Context) {
	res, _ := c.AssetService.GetTeamAvatarList()
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": res,
	})
}

// GetTeamAvatarByTeamId
// @Summary 通过团队 Id 获取团队头像
// @Description 通过团队 Id 获取团队头像
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "团队 Id"
// @Router /api/assets/teams/avatar/{id} [get]
func (c *AssetControllerImpl) GetTeamAvatarByTeamId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("%s/teams/avatar/%s", assetsPath, id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.File(path)
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
	}
}

// GetTeamAvatarInfoByTeamId
// @Summary 通过团队 Id 获取团队头像信息
// @Description 通过团队 Id 获取团队头像信息
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "团队 Id"
// @Router /api/assets/teams/avatar/{id}/info [get]
func (c *AssetControllerImpl) GetTeamAvatarInfoByTeamId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("%s/teams/avatar/%s", assetsPath, id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
	}
}

// SetTeamAvatarByTeamId
// @Summary 通过团队 Id 设置团队头像
// @Description 通过团队 Id 设置团队头像
// @Tags 资源
// @Accept multipart/form-data
// @Param id path string true "团队 Id"
// @Param avatar formData file true "头像文件"
// @Router /api/assets/teams/avatar/{id} [post]
func (c *AssetControllerImpl) SetTeamAvatarByTeamId(ctx *gin.Context) {
	id := ctx.Param("id")
	file, err := ctx.FormFile("avatar")
	if err != nil {
		ctx.Status(http.StatusBadRequest)
		return
	}
	mime, err := c.detectContentType(file)
	if !mime.Is("image/jpeg") && !mime.Is("image/png") && !mime.Is("image/gif") {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "格式不被允许",
		})
		return
	}
	err = ctx.SaveUploadedFile(file, fmt.Sprintf("%s/teams/avatar/%s", assetsPath, id))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// DeleteTeamAvatarByTeamId
// @Summary 通过团队 Id 删除团队头像
// @Description 通过团队 Id 删除团队头像
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "用户 Id"
// @Router /api/assets/teams/avatar/{id} [delete]
func (c *AssetControllerImpl) DeleteTeamAvatarByTeamId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("%s/teams/avatar/%s", assetsPath, id)
	_, err := os.Stat(path)
	if err == nil {
		err = os.Remove(path)
		if err != nil {
			ctx.JSON(http.StatusOK, gin.H{
				"code": http.StatusInternalServerError,
				"msg":  err.Error(),
			})
			return
		}
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
	}
}

// GetGameCoverByGameId
// @Summary 通过比赛 Id 获取比赛封面
// @Description 通过比赛 Id 获取比赛封面
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "比赛 Id"
// @Router /api/assets/games/cover/{id} [get]
func (c *AssetControllerImpl) GetGameCoverByGameId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("%s/games/cover/%s", assetsPath, id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.File(path)
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
	}
}

// SetGameCoverByGameId
// @Summary 通过比赛 Id 设置比赛封面
// @Description 通过比赛 Id 设置比赛封面
// @Tags 资源
// @Accept multipart/form-data
// @Param id path string true "比赛 Id"
// @Param avatar formData file true "封面文件"
// @Router /api/assets/games/cover/{id} [post]
func (c *AssetControllerImpl) SetGameCoverByGameId(ctx *gin.Context) {
	id := ctx.Param("id")
	file, err := ctx.FormFile("avatar")
	if err != nil {
		ctx.Status(http.StatusBadRequest)
		return
	}
	mime, err := c.detectContentType(file)
	if !mime.Is("image/jpeg") && !mime.Is("image/png") && !mime.Is("image/gif") {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "格式不被允许",
		})
		return
	}
	err = ctx.SaveUploadedFile(file, fmt.Sprintf("%s/games/cover/%s", assetsPath, id))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// FindGameWriteUpByTeamId
// @Summary 通过团队 Id 获取比赛 Writeup
// @Description 通过团队 Id 获取比赛 Writeup
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "团队 Id"
// @Router /api/assets/games/writeups/{id} [get]
func (c *AssetControllerImpl) FindGameWriteUpByTeamId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("%s/games/writeups/%s.pdf", assetsPath, id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.File(path)
	} else {
		ctx.Status(http.StatusNotFound)
	}
}

// SetChallengeAttachmentByChallengeId
// @Summary 通过题目 Id 设置题目附件
// @Description 通过题目 Id 设置题目附件
// @Tags 资源
// @Accept multipart/form-data
// @Param id path string true "题目 Id"
// @Param attachment formData file true "附件文件"
// @Router /api/assets/challenges/attachments/{id} [post]
func (c *AssetControllerImpl) SetChallengeAttachmentByChallengeId(ctx *gin.Context) {
	id := ctx.Param("id")
	file, err := ctx.FormFile("attachment")
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "无效文件",
		})
		return
	}
	if _, fileSize, _ := c.AssetService.CheckChallengeAttachmentByChallengeId(int64(convertor.ToIntD(id, 0))); fileSize != 0 {
		err = c.AssetService.DeleteChallengeAttachmentByChallengeId(int64(convertor.ToIntD(id, 0)))
		if err != nil {
			ctx.JSON(http.StatusOK, gin.H{
				"code": http.StatusInternalServerError,
				"msg":  err.Error(),
			})
			return
		}
	}
	err = ctx.SaveUploadedFile(file, fmt.Sprintf("%s/challenges/attachments/%s/%s", assetsPath, id, file.Filename))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// GetChallengeAttachmentInfoByChallengeId
// @Summary 通过题目 Id 查找题目附件信息
// @Description 通过题目 Id 查找题目附件信息
// @Tags 资源
// @Accept json
// @Param id path string true "题目 Id"
// @Router /api/assets/challenges/attachments/{id}/info [get]
func (c *AssetControllerImpl) GetChallengeAttachmentInfoByChallengeId(ctx *gin.Context) {
	id := ctx.Param("id")
	fileName, fileSize, err := c.AssetService.CheckChallengeAttachmentByChallengeId(int64(convertor.ToIntD(id, 0)))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":      http.StatusOK,
		"file_name": fileName,
		"file_size": fileSize,
	})
}

// GetChallengeAttachmentByChallengeId
// @Summary 通过题目 Id 获取题目附件
// @Description 通过题目 Id 获取题目附件
// @Tags 资源
// @Accept json
// @Param id path string true "题目 Id"
// @Router /api/assets/challenges/attachments/{id} [get]
func (c *AssetControllerImpl) GetChallengeAttachmentByChallengeId(ctx *gin.Context) {
	id := ctx.Param("id")
	fileName, _, err := c.AssetService.CheckChallengeAttachmentByChallengeId(int64(convertor.ToIntD(id, 0)))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
		return
	}
	ctx.File(fmt.Sprintf("%s/challenges/attachments/%s/%s", assetsPath, id, fileName))
}

// DeleteChallengeAttachmentByChallengeId
// @Summary 通过题目 Id 删除 题目附件
// @Description 通过题目 Id 删除题目附件
// @Tags 资源
// @Accept json
// @Param id path string true "题目 Id"
// @Router /api/assets/challenges/attachments/{id} [delete]
func (c *AssetControllerImpl) DeleteChallengeAttachmentByChallengeId(ctx *gin.Context) {
	id := ctx.Param("id")
	err := c.AssetService.DeleteChallengeAttachmentByChallengeId(int64(convertor.ToIntD(id, 0)))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

func (c *AssetControllerImpl) detectContentType(file *multipart.FileHeader) (mime *mimetype.MIME, err error) {
	src, err := file.Open()
	if err != nil {
		return nil, err
	}
	defer func(src multipart.File) {
		_ = src.Close()
	}(src)
	buffer := make([]byte, 512)
	_, err = src.Read(buffer)
	if err != nil {
		return nil, err
	}
	return mimetype.Detect(buffer), nil
}
