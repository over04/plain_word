export default {
  common: {
    appName: 'Plain Word',
    loading: '加载中...',
    save: '保存',
    cancel: '取消',
    delete: '删除',
    edit: '编辑',
    create: '创建',
    confirm: '确认',
    back: '返回',
    clear: '清除',
    close: '关闭',
    search: '搜索',
    noData: '暂无数据',
    success: '操作成功',
    error: '操作失败',
    required: '必填',
    optional: '选填'
  },
  errors: {
    UNAUTHORIZED: '请先登录',
    INVALID_CREDENTIALS: '用户名或密码错误',
    VALIDATION_ERROR: '输入验证失败',
    NOT_FOUND: '资源不存在',
    CONFLICT: '资源冲突',
    DATABASE_ERROR: '数据库错误',
    INTERNAL_ERROR: '服务器内部错误',
    UNKNOWN: '未知错误'
  },
  nav: {
    home: '首页',
    wordbooks: '单词本',
    tags: '标签',
    settings: '设置',
    login: '登录',
    register: '注册',
    logout: '退出登录'
  },
  home: {
    title: 'Plain Word',
    subtitle: '一本精心设计的单词本，让背单词像书写一样自然。用你自己的方式，自然地学习词汇。',
    getStarted: '开始使用',
    signIn: '立即登录',
    features: {
      wordbooks: {
        title: '多本单词本',
        desc: '按主题、难度或任何你喜欢的方式组织词汇。'
      },
      tags: {
        title: '灵活标签',
        desc: '自由标记单词，筛选后专注学习。'
      },
      modes: {
        title: '学习模式',
        desc: '在原文、译文和双语视图间自由切换。'
      }
    }
  },
  auth: {
    login: {
      title: '欢迎回来',
      subtitle: '登录以继续学习',
      username: '用户名',
      usernamePlaceholder: '请输入用户名',
      password: '密码',
      passwordPlaceholder: '请输入密码',
      submit: '登录',
      submitting: '登录中...',
      noAccount: '还没有账号？',
      signUp: '立即注册'
    },
    register: {
      title: '创建账号',
      subtitle: '开启你的词汇学习之旅',
      username: '用户名',
      usernamePlaceholder: '请选择一个用户名',
      email: '邮箱',
      emailPlaceholder: '请输入邮箱地址',
      password: '密码',
      passwordPlaceholder: '请设置密码',
      confirmPassword: '确认密码',
      confirmPasswordPlaceholder: '请再次输入密码',
      submit: '创建账号',
      submitting: '创建中...',
      hasAccount: '已有账号？',
      signIn: '立即登录',
      passwordMismatch: '两次输入的密码不一致'
    }
  },
  wordbooks: {
    title: '我的单词本',
    new: '新建单词本',
    empty: {
      title: '还没有单词本',
      desc: '创建你的第一本单词本开始学习吧',
      action: '创建单词本'
    },
    deleteConfirm: '确定要删除这本单词本吗？',
    modal: {
      createTitle: '新建单词本',
      editTitle: '编辑单词本',
      name: '名称',
      namePlaceholder: '例如：托福词汇',
      description: '描述',
      descriptionPlaceholder: '添加描述（选填）...',
      create: '创建'
    },
    createdAt: '创建于'
  },
  chapters: {
    title: '章节',
    add: '添加章节',
    backTo: '返回',
    empty: {
      title: '还没有章节',
      desc: '添加章节来组织你的词汇',
      action: '添加第一个章节'
    },
    deleteConfirm: '确定要删除这个章节吗？',
    modal: {
      createTitle: '添加新章节',
      name: '章节名称',
      namePlaceholder: '例如：第一周 - 基础词汇',
      create: '添加章节'
    }
  },
  words: {
    title: '单词',
    add: '添加单词',
    controls: '控制面板',
    display: '显示',
    displayModes: {
      original: '原文',
      translation: '译文',
      bilingual: '双语'
    },
    shuffle: '打乱',
    filterByTag: '按标签筛选：',
    empty: {
      title: '还没有单词',
      desc: '开始构建你的词汇库',
      action: '添加第一个单词'
    },
    filterEmpty: {
      title: '没有匹配的单词',
      desc: '当前标签筛选没有找到单词'
    },
    deleteConfirm: '确定要删除这个单词吗？',
    clickToReveal: '点击显示',
    modal: {
      createTitle: '添加新单词',
      editTitle: '编辑单词',
      source: '原文',
      sourcePlaceholder: '输入原文',
      translation: '译文',
      translationPlaceholder: '输入译文',
      note: '备注',
      notePlaceholder: '添加备注...',
      create: '添加单词',
      save: '保存'
    },
    manageTags: '管理标签',
    tags: {
      add: '添加标签',
      remove: '点击移除',
      noMore: '没有更多标签了'
    },
    manage: {
      title: '管理单词',
      view: '学习视图',
      table: '管理视图',
      selectAll: '全选',
      selected: '已选择 {count} 项',
      batchDelete: '批量删除',
      batchEditTags: '批量编辑标签',
      deleteConfirm: '确定要删除选中的 {count} 个单词吗？',
      noSelection: '请先选择单词',
      searchPlaceholder: '搜索单词...',
      addTag: '添加标签',
      removeTag: '移除标签',
      tagEditHint: '点击添加或移除标签'
    }
  },
  tags: {
    title: '标签管理',
    new: '新建标签',
    empty: {
      title: '还没有标签',
      desc: '创建标签来组织你的词汇',
      action: '创建第一个标签'
    },
    deleteConfirm: '确定要删除这个标签吗？',
    modal: {
      createTitle: '创建新标签',
      name: '标签名称',
      namePlaceholder: '例如：重点、复习、简单',
      color: '颜色',
      create: '创建'
    }
  },
  importExport: {
    importWordbook: '导入单词本',
    importChapter: '导入章节',
    exportWordbook: '导出单词本',
    exportChapter: '导出章节',
    downloadTemplate: '下载模板',
    selectFile: '选择文件',
    selectFormat: '选择格式',
    formats: {
      json: 'JSON',
      xml: 'XML',
      xlsx: 'Excel',
      csv: 'CSV'
    },
    name: '名称',
    namePlaceholder: '输入名称...',
    import: '导入',
    importing: '导入中...',
    success: '导入成功',
    chaptersCreated: '创建章节数',
    wordsCreated: '创建单词数'
  },
  settings: {
    title: '设置',
    account: {
      title: '账号信息',
      username: '用户名',
      email: '邮箱',
      memberSince: '注册时间'
    },
    display: {
      title: '显示设置'
    },
    lineHeight: {
      title: '行高'
    },
    wordSpacing: {
      title: '单词间距'
    },
    about: {
      title: '关于 Plain Word',
      desc: 'Plain Word 是一款精心设计的单词本应用，旨在让英语词汇学习变得自然而直观，就像在纸质笔记本上书写一样。',
      version: '版本'
    },
    language: {
      title: '语言设置',
      current: '当前语言'
    }
  }
}